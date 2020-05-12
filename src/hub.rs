use futures::{Stream, TryStreamExt};
use tokio::sync::broadcast;
use tokio::time;

use crate::error::Result;
use crate::proto::{InputMessage, Output, OutputMessage};

const OUTPUT_CHANNEL_SIZE: usize = 3;
const ALIVE_INTERVAL: time::Duration = time::Duration::from_secs(1);

pub struct Hub {
    output_sender: broadcast::Sender<OutputMessage>,
    // feed: Arc<Mutex<Feed>>,
}

impl Hub {
    pub fn new() -> Self {
        let (output_sender, _) = broadcast::channel(OUTPUT_CHANNEL_SIZE);
        Hub {
            output_sender,
            // feed: Arc::new(Mutex::new(Feed::new())),
        }
    }

    pub async fn run<S>(&self, stream: S) -> Result<()>
    where
        S: Stream<Item = Result<InputMessage>>,
    {
        let ticking_alive = self.tick_alive();

        let processing = stream.try_for_each(|_m| async {
            // self.send(OutputMessage::new(Output::Alive));
            Ok(())
        });

        tokio::select! {
            result = ticking_alive => result,
            result = processing => result,
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<OutputMessage> {
        self.output_sender.subscribe()
    }

    async fn tick_alive(&self) -> Result<()> {
        let mut interval = time::interval(ALIVE_INTERVAL);
        loop {
            interval.tick().await;
            if self.output_sender.receiver_count() > 0 {
                self.output_sender
                    .send(OutputMessage::new(Output::Alive))
                    .unwrap();
            }
        }
    }

    fn send(&self, message: OutputMessage) {
        if self.output_sender.receiver_count() > 0 {
            self.output_sender.send(message).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sign_in() {}
}
