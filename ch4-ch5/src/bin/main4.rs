#[derive(Debug)]
struct FileDownload {
    name: String,
    total_size: u32,
    downloaded: u32,
    is_completed: bool,
}

impl FileDownload {
    fn new(name: String, size: u32) -> FileDownload {
        FileDownload {
            name,
            total_size: size,
            downloaded: 0,
            is_completed: false,
        }
    }

    fn show_progress(&self) {
        let percentage = (self.downloaded as f32 / self.total_size as f32) * 100.0;
        println!(
            "File: {} | Progress: {}/{} MB ({:.1}%) | Completed: {}",
            self.name, self.downloaded, self.total_size, percentage, self.is_completed
        );
    }

    fn receive_data(&mut self, amount: u32) {
        if self.is_completed {
            println!("Error: File '{}' is already finished.", self.name);
            return;
        }

        self.downloaded += amount;

        if self.downloaded >= self.total_size {
            self.downloaded = self.total_size;
            self.is_completed = true;
            println!("Download completed: {}", self.name);
        } else {
            println!("Received {} MB...", amount);
        }
    }
}

fn main() {
    let mut my_file = FileDownload::new(String::from("kali_linux.iso"), 1000);

    my_file.show_progress();

    println!("--- Start Downloading ---");

    my_file.receive_data(250);
    my_file.show_progress();

    my_file.receive_data(500);
    my_file.show_progress();

    my_file.receive_data(300);
    my_file.show_progress();

    my_file.receive_data(50);
}