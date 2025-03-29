pub fn update(&mut self, message: Message) -> Command<Message> {
    match message {
        // ... existing code ...
        Message::ChannelsSearchChanged(search) => {
            self.channels_search = search;
            Command::none()
        }
        Message::PlayChannel(url) => {
            if let Err(e) = open::that(&url) {
                eprintln!("Error al abrir la URL del canal: {}", e);
            }
            Command::none()
        }
        // ... existing code ...
    }
} 