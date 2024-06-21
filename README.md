# VR-scrcpy
Small tauri app to manage scrcpy and launching apps on multiple android based VR (oculus 2 and 3)
<br>
# How to isntall
Download the exe, add a src folder next to it with the text files for ip_list.txt and processes.txt.
<br>
Make user you have scrcpy and adb in ENV variables. 
Launch app.
# How to use
1. Start ADB server by clicking START and kill-server by clicking STOP
2. Clicking connect will iterate through the ip_list.txt, attempt to run adb connect {ip} and timeout after 3 seconds.
3. Click Get all to get connected devices. They should now appear in the list.
4. Select any devices you wish to run the commands on
5. Click buttons on the right of the list to run commands:
   - Select process (items taken from processes list) and click Launch/Close
   - Disconnect will run adb disconnect {ip}
   - Launch SCRCPY will launch an instance of scrcpy with basic settings.
