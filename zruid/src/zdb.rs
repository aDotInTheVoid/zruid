use std::sync::Arc;

use futures::future::{join, join_all};

// TODO: Move all this to zdb crate, and persist. And less lineear scans

const ZOE_API_KEY: &str = "XNKVLPJXdCHDp2WcXBGHWtAVAYjZIMVJ";
const ZOE_EMAIL: &str = "ZOE@zulip.com";
const DEV_SERVER: &str = "localhost:9991";

#[derive(Debug, Clone, PartialEq)]
pub struct State {
    streams: Vec<Stream>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Stream {
    id: u64,
    topics: Vec<Topic>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Topic {
    name: String,
    messages: Vec<Message>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    from: String,
    content: String,
    id: u64,
}

// Spin up a RT to get the state
pub fn state() -> State {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(get_state())
}

async fn get_state() -> State {
    let c = zrest::Client::new(ZOE_EMAIL, ZOE_API_KEY, DEV_SERVER);
    let c = Arc::new(c);

    let streams = c.streams().await.unwrap().streams;

    let all_streams = streams.into_iter().map(|stream| {
        let c = Arc::clone(&c);
        tokio::spawn(async move {
            let topics = c.topics(stream.stream_id).await?.topics;

            let stream = Stream {
                topics: topics
                    .into_iter()
                    .map(|t| Topic {
                        messages: Vec::new(),
                        name: t.name,
                    })
                    .collect(),
                id: stream.stream_id,
            };

            // TODO: Use eyre
            Result::<_, Box<dyn std::error::Error + Send + Sync + 'static>>::Ok(stream)
        })
    });

    let all_streams = join_all(all_streams);

    let all_messages = c.messages();

    let (streams, messages) = join(all_streams, all_messages).await;

    let mut streams = streams
        .into_iter()
        .map(|x| x.unwrap().unwrap())
        .collect::<Vec<_>>();

    for i in messages.unwrap().messages {
        // FIXME: Dont ignore DMs
        if let Some(sid) = i.stream_id {
            let stream_in = streams.iter_mut().find(|s| s.id == sid).unwrap();
            let topic_in = stream_in
                .topics
                .iter_mut()
                .find(|t| t.name == i.topic)
                .unwrap();

            topic_in.messages.push(Message {
                content: i.content,
                id: sid,
                from: i.sender_full_name,
            });
        }
    }

    State { streams }
}
