# **📌 Project Overview**

A Pomodoro timer follows the **Pomodoro Technique**, which consists of:

- **25 minutes** of focused work
- **5-minute break**
- Repeat this cycle **4 times**, then take a longer break (e.g., 15-30 minutes).

This **command-line program** that allows users to:  
✅ Start a Pomodoro session  
✅ Customize work and break intervals  
✅ Display time remaining in the terminal  
✅ Play a sound or show a notification when time is up

---

## **🛠 Features**

1. **Basic Timer**
    
    - Use `std::thread::sleep()` to count down time.
    - Print time remaining every second.

2. **CLI Arguments for Customization**
    
    - Use `clap` crate to allow users to set custom durations.
    - Example:
        
```bash   
$ pomodoro --work 30 --break 10
```
        
3. **Loop for Multiple Sessions**
    
    - Run the Pomodoro cycle 4 times before a long break.
    - Use a simple loop with counters.
4. **Terminal UI (Optional but Fun!)**
    
    - Improve UI with `crossterm` or `ratatui` (formerly `tui`).
    - Show a progress bar or countdown animation.
5. **Sound or Notification (Optional)**
    
    - Use `rodio` crate to play a sound when time is up.
    - Send desktop notifications using `notify-rust`.

---

## **🖥 Example Usage**

```bash
$ pomodoro --work 25 --break 5 --sessions 4 
Pomodoro session started!  
[██████████               ]  15:00 left... 
[     ] 05:00 left... 
... 
Time's up! Take a break! 🎉
```
