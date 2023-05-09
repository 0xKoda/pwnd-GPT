
use std::env;
use std::io;

use indicatif::{ProgressBar, ProgressStyle};
use llm_chain::traits::Executor;
use llm_chain::{executor, parameters, prompt, step::Step};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ANSI color code for texts
    let green = "\u{001b}[32m";
    let blue = "\u{001b}[34m";
    // Reset ANSI color code
    let reset = "\u{001b}[0m";

    println!(
        "{}{}{}",
        green,
        r#"
                             __ _______ _______ _______ 
    .-----.--.--.--.-----.--|  |   _   |   _   |       |
    |  _  |  |  |  |     |  _  |.  |___|.  1   |.|   | |
    |   __|________|__|__|_____|.  |   |.  ____`-|.  |-'
    |__|                       |:  1   |:  |     |:  |  
                               |::.. . |::.|     |::.|  
                               `-------`---'     `---'     
    "#,
        reset
    );

    println!(
        "{}{}{}",
        blue,
        r#"
        Helping you deal with Cyber Incident Response  
    "#,
        reset
    );

    loop {
        std::env::var("OPENAI_API_KEY").is_ok();

        let exec = executor!()?;

        let step = Step::for_prompt_template(prompt!(
            "You are acting as a seniour specialist agent in cybersecurity incident response.
        You have an in depth knowledge of how to investigate cyber incidents.
        A juniour developer comes to you with this security issue {{user_request}}
        and they have these tools installed {{cli_tool}}. Help the user respond to the incident
        guiding them through the incident response process, and providing any command line tool
        terminal commands.  If {{cli_tool}} is none, guide the user through the commands to use any default cli tools that might be useful.
        Your response has to be only the following YAML template:

        ```yaml
        explanation: Your brief explanation goes here.
        command: {{cli_tool}} command solution goes here.
        ```

        Only replace the information in the YAML template above
        and only return that YAML template as your answer.",
            "{{user_request}}"
        ));

        let mut user_request = String::new();
        println!("Describe your issue:");
        io::stdin()
            .read_line(&mut user_request)
            .expect("error: unable to read user input");

        let mut cli_tool = String::new();
        println!("What CLI tools do you have?");
        io::stdin()
            .read_line(&mut cli_tool)
            .expect("error: unable to read user input");

        let params = parameters!(
            "cli_tool" => cli_tool.trim(),
            "user_request" => user_request.trim()
        );

        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                .template("{spinner:.green} Processing..."),
        );
        pb.enable_steady_tick(100);

        let res = step.run(&params, &exec).await?;

        pb.finish_and_clear();

        println!("{}{}{}", green, res, reset);

        println!("Do you need more help? (yes/no):");
        let mut more_help = String::new();
        io::stdin()
            .read_line(&mut more_help)
            .expect("error: unable to read user input");

        if more_help.trim().to_lowercase() != "yes" {
            break;
        }
   
    }

    Ok(())
    }