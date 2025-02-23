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
    
    [x] Use `std::thread::sleep()` to count down time.
    [x] Print time remaining every second.

2. **CLI Arguments for Customization**
    
    [x] Use `clap` crate to allow users to set custom durations.
    - Example:
        
```bash   
$ pomodoro --work 30 --rest 10 --sessions 3
```
        
3. **Loop for Multiple Sessions**
    
    [x] Run the Pomodoro cycle x times

4. **Terminal UI (Optional but Fun!)**
    
    [x] Improve UI with `crossterm`.
    [x] Show a progress bar or countdown animation.

5. **Sound or Notification (Optional)**
    
    [ ] Use `rodio` crate to play a sound when time is up.
    [ ] Send desktop notifications using `notify-rust`.

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
