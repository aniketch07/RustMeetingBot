use std::error::Error;
use thirtyfour::prelude::*; // Selenium WebDriver crate
use tokio::runtime::Runtime; // Tokio runtime
use tokio::time::Duration; // For sleep
use url::Url; // For URL parsing

// Start browser session, navigate to Google Meet, enter the name, and click buttons
async fn spawn_browser() -> Result<(), Box<dyn Error>> {
    // Set up Chrome capabilities
    let mut caps = DesiredCapabilities::chrome();
    caps.add_arg("--disable-blink-features=AutomationControlled")?;
    caps.add_arg("--use-fake-ui-for-media-stream")?;

    // Start WebDriver session
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    println!("Browser started successfully!");

    // Navigate to Google Meet
    let url = Url::parse("https://meet.google.com/hkg-mrrd-qin")?;
    driver.get(url.as_str()).await?;
    println!("Navigated to Google Meet!");

    // Wait and input name
    let input_elem = driver
        .query(By::Css(r#"input[placeholder="Your name"]"#))
        .wait(Duration::from_secs(5), Duration::from_secs(1))
        .first()
        .await?;
    input_elem.send_keys("Aniket-BOT").await?;
    println!("Name entered successfully!");

    // Click 'Got it' button
    let got_it_btn = driver
        .query(By::XPath(r#"//button[span[text()='Got it']]"#))
        .wait(Duration::from_secs(5), Duration::from_secs(1))
        .first()
        .await?;
    got_it_btn.click().await?;
    println!("Clicked 'Got it' button successfully!");

    // Click 'Ask to join' button
    let join_btn = driver
        .query(By::XPath(r#"//button[span[text()='Ask to join']]"#))
        .wait(Duration::from_secs(5), Duration::from_secs(1))
        .first()
        .await?;
    join_btn.click().await?;
    println!("Clicked 'Ask to join' button successfully!");

    // Wait and observe
    tokio::time::sleep(Duration::from_secs(50)).await;

    // Close the browser session
    driver.quit().await?;
    println!("Browser closed successfully!");

    Ok(())
}

fn main() {
    // Create a new Tokio runtime and run the asynchronous function
    let runtime = Runtime::new().expect("Failed to create Tokio runtime");
    runtime.block_on(async {
        if let Err(e) = spawn_browser().await {
            eprintln!("Error: {}", e);
        }
    });

    println!("Process completed!");
}

