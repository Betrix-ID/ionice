#!/system/bin/sh
 if [ "$(id -u)" -ne 0 ] && [ "$(id -u)" -ne 2000 ]; then
	printf "[ Eror |@Yeye_nat(Yeye)]\n"
	exit 1
 fi
 
#Chking cpu.abi
     if [ ! -f /sdcard/ionik/OI/target ]; then
	    architecture=$(getprop ro.product.cpu.abi)
	  if [ "$architecture" = "arm64-v8a" ]; then
		cp /sdcard/ionik/OI/target/release/arm64 /sdcard/ionik/pidof
    else 
         printf "Scripting ini tidak mendukung cpu.abi anda : $architecture \n"
      fi
   fi
#Smart notification
   shell() {
     cont="$1"
     cmd notification post -S bigtext -t '♨️ Multicor Priority ' 'Tag' "$cont" > /dev/null 2>&1 
   }
set +x
    echo
    echo "     ☆================================☆"
    echo
    echo "          ~ Description. Multicor Priority...... "
    echo
    echo "           - Author                  :  @UnixeID"
    echo "           - Point                     :  1.0 "
    echo "           - Release                :  15 - Mei - 2025"
    echo "           - Name Shell          :   Multicor Priority"
    echo
    echo "    |_______________________________________|"
    echo "    \______________________________________/"
    echo
    echo "   Priority Multicor Priority Custem. "
    sleep 2
    echo
        # Ambil daftar paket
      package_list=$(pm list package | cut -f2 -d: | tr -d '\r' | xargs -n1)
     control=1
       while IFS= read -r gamelist || [ -n "$gamelist" ]; do
          line=$(echo "$gamelist" | tr -d '\r' | xargs)
              if [ -n "$line" ]; then
        if echo "$package_list" | grep -xq "$line"; then
            echo "  $control. $line"
            control=$((control + 1))
          else
            echo "Paket game '$line' tidak ditemukan."
              fi
                fi
            done < /sdcard/ionik/GamePid.txt
          # Buat varibale instlangi toast.apk
          if [ -f /sdcard/ionik/toast.apk ]; then
    if pm list package | cut -f2 -d: | grep bellavita.toast; then
           echo
     else       
         cp /sdcard/ionik/toast.apk /data/local/tmp
          pm install /data/local/tmp/toast.apk
       fi
     fi > /dev/null 2>&1       
     # Buat varibale instlling and uninstalling script
  if [ "$1" = kill ]; then
        if pgrep -f pidof > /dev/null 2>&1; then
         echo "  Program is stopped in the backgurond "
         pm uninstall bellavita.toast > /dev/null 2>&1
         rm /data/local/tmp/pidof > /dev/null 2>&1
         pkill -f pidof > /dev/null 2>&1
         shell "Program is stopped in the backgurond" 
         pkill -f sh > /dev/null 2>&1       
     else
       echo "Porgam faild stop !"
   fi
  else
     if ! pgrep -f pidof > /dev/null 2>&1; then
       cp /sdcard/ionik/pidof /data/local/tmp
       chmod +x /data/local/tmp/pidof
      nohup /data/local/tmp/pidof > /dev/null 2>&1
    fi 
      sleep 2
        if pgrep -f pidof > /dev/null 2>&1; then
        echo " Program is running in the backgurond"
     else
          echo "Porgram faild running !"
     fi
  fi 
set +x