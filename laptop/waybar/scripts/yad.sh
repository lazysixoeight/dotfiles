#!/usr/bin/env bash

while [[ $# -gt 0 ]]; do
  case "$1" in
    --poweroff)
      yad --image="dialog-warning" \
        --title "Confirmation" \
        --text "Are you sure you want to poweroff?" \
        --button="yad-yes:0" \
        --button="yad-no:1"
      ret=$?
      [[ $ret -eq 0 ]] && poweroff
      [[ $ret -eq 1 ]] && echo "Not shutting down" & exit 1
      ;;
    --reboot)
      yad --image="dialog-warning" \
        --title "Confirmation" \
        --text "Are you sure you want to reboot?" \
        --button="yad-yes:0" \
        --button="yad-no:1"
      ret=$?
      [[ $ret -eq 0 ]] && reboot
      [[ $ret -eq 1 ]] && echo "Not rebooting" & exit 1
      ;;
    --suspend)
      yad --image="dialog-warning" \
        --title "Confirmation" \
        --text "Are you sure you want to suspend?" \
        --button="yad-yes:0" \
        --button="yad-no:1"
      ret=$?
      [[ $ret -eq 0 ]] && systemctl suspend
      [[ $ret -eq 1 ]] && echo "Not suspending" & exit 1
      ;;
  esac
done
