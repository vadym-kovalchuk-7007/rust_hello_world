#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Watching the {:?}", self.content);
    }
}

fn main() {
    let chat_msg = ChatMessage {
        content: DigitalContent::AudioFile,
        time: String::from("120"),
    };
    let chat_msg_time = chat_msg.retrieve_time();
    println!("Chat mes audio {}", chat_msg.retrieve_time());
    chat_msg.consume_entertainment();
    let chat_msg_str = ChatMessage {
        content: String::from("Vidos"),
        time: String::from("130"),
    };
    println!(
        "Chat mes {} {}",
        &chat_msg_str.content,
        chat_msg_str.retrieve_time()
    );

    let chat_msg_str2 = ChatMessage {
        content: "Audios",
        time: String::from("140"),
    };
    println!(
        "Chat mes {} {}",
        &chat_msg_str2.content,
        chat_msg_str2.retrieve_time()
    );
}
