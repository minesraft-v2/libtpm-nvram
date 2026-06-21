#!/bin/sh

BLUE='\033[0;34m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
RESET='\033[0m'

clear
echo -e "${BLUE}====================================================${RESET}"
echo -e "${CYAN}             VNV-SWITCHER DASHBOARD                 ${RESET}"
echo -e "${BLUE}====================================================${RESET}"
echo -e "Developed by: ${GREEN}minesraft-v2${RESET}\n"

echo -e "Select a target kernel configuration level:"
echo -e " Target Level 0 (Legacy Stable)"
echo -e " Target Level 1"
echo -e " Target Level 2"
echo -e " Target Level 3"
echo -e " Target Level 4"
echo -e " Target Level 5 (Latest Kernel)"
echo -e " Exit Utility"
echo ""
echo -n "Enter your choice (1-7): "
read choice

case $choice in
    1) TARGET="0" ;;
    2) TARGET="1" ;;
    3) TARGET="2" ;;
    4) TARGET="3" ;;
    5) TARGET="4" ;;
    6) TARGET="5" ;;
    7) echo -e "${YELLOW}Exiting switcher.${RESET}"; exit 0 ;;
    *) echo -e "${YELLOW}Invalid option selection.${RESET}"; exit 1 ;;
esac

echo -e "\n${CYAN}[*] Handing execution over to core binary...${RESET}"
./target/release/vnv-switcher --target "$TARGET"
