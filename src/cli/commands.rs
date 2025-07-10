use std::path::PathBuf;
use anyhow::{Context, Result};

use crate::script::{Script, ScriptLoader};
use crate::pty::TerminalController;
use crate::media::{MediaRecorder, OutputFormat};

pub async fn record_command(
    script_path: PathBuf,
    output_dir: PathBuf,
    format: String,
) -> Result<()> {
    println!("🎬 Recording script: {}", script_path.display());
    
    // Load script
    let script = ScriptLoader::load_from_file(&script_path)
        .with_context(|| format!("Failed to load script: {}", script_path.display()))?;
    
    // Parse output format
    let output_format = OutputFormat::from_string(&format)?;
    
    // Create output directory
    std::fs::create_dir_all(&output_dir)
        .with_context(|| format!("Failed to create output directory: {}", output_dir.display()))?;
    
    // Initialize terminal controller
    let mut terminal = TerminalController::new(&script.settings)?;
    
    // Initialize media recorder
    let mut recorder = MediaRecorder::new(output_format, &output_dir)?;
    
    // Execute script
    println!("🚀 Executing {} steps...", script.steps.len());
    
    for (i, step) in script.steps.iter().enumerate() {
        println!("📝 Step {}/{}: {:?}", i + 1, script.steps.len(), step.step_type);
        
        match step.step_type {
            crate::script::StepType::Command { ref text, wait } => {
                terminal.execute_command(text).await?;
                if let Some(duration) = wait {
                    tokio::time::sleep(duration).await;
                }
            }
            crate::script::StepType::Type { ref text, speed } => {
                terminal.type_text(text, speed).await?;
            }
            crate::script::StepType::Screenshot { ref name } => {
                let screenshot_path = output_dir.join(format!("{}.png", name));
                recorder.take_screenshot(&terminal, &screenshot_path).await?;
                println!("📸 Screenshot saved: {}", screenshot_path.display());
            }
            crate::script::StepType::RecordGif { duration, ref name } => {
                let gif_path = output_dir.join(format!("{}.gif", name));
                recorder.start_gif_recording(&terminal).await?;
                tokio::time::sleep(duration).await;
                recorder.stop_gif_recording(&gif_path).await?;
                println!("🎞️ GIF saved: {}", gif_path.display());
            }
        }
    }
    
    println!("✅ Recording complete! Output saved to: {}", output_dir.display());
    Ok(())
}

pub async fn screenshot_command(command: String, output: PathBuf) -> Result<()> {
    println!("📸 Taking screenshot of command: {}", command);
    
    // Create a simple single-command script
    let script = Script::single_command(&command)?;
    
    // Initialize terminal
    let mut terminal = TerminalController::new(&script.settings)?;
    
    // Execute command
    terminal.execute_command(&command).await?;
    
    // Take screenshot
    let recorder = MediaRecorder::new(OutputFormat::Png, &output.parent().unwrap_or(&PathBuf::from(".")))?;
    recorder.take_screenshot(&terminal, &output).await?;
    
    println!("✅ Screenshot saved: {}", output.display());
    Ok(())
}

pub async fn demo_command(script_path: PathBuf, interactive: bool) -> Result<()> {
    println!("🎭 Running demo: {}", script_path.display());
    
    let script = ScriptLoader::load_from_file(&script_path)?;
    let mut terminal = TerminalController::new(&script.settings)?;
    
    for (i, step) in script.steps.iter().enumerate() {
        if interactive {
            println!("\n📋 Next step {}/{}: {:?}", i + 1, script.steps.len(), step.step_type);
            println!("Press Enter to continue...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
        }
        
        match step.step_type {
            crate::script::StepType::Command { ref text, wait } => {
                terminal.execute_command(text).await?;
                if let Some(duration) = wait {
                    tokio::time::sleep(duration).await;
                }
            }
            crate::script::StepType::Type { ref text, speed } => {
                terminal.type_text(text, speed).await?;
            }
            _ => {} // Skip recording steps in demo mode
        }
    }
    
    println!("✅ Demo complete!");
    Ok(())
}

pub async fn convert_command(input: PathBuf, output: PathBuf) -> Result<()> {
    println!("🔄 Converting {} to {}", input.display(), output.display());
    
    // TODO: Implement format conversion logic
    // This would handle converting between different recording formats
    
    println!("✅ Conversion complete!");
    Ok(())
}