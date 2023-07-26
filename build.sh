#!/bin/bash

declare -xr NC="\033[0m"
declare -xr RED="\033[31m"
declare -xr GREEN="\033[32m"
declare -xr YELLOW="\033[93m"

declare ARGS_CONFIG=""

parse_args() {
  while [ "$#" -gt 0 ]; do
    case "$1" in
      -c | --config) shift; ARGS_CONFIG=$1 ;;
    esac
    shift
  done
}

repeat(){
	for i in {1..90}; do echo -en "$1"; done
}

########################################
# Run the command passed as 1st argument and 
# shows the spinner until this is done
# Arguments:
#   $1 the command to run
#   $2 the title to show next to the spinner
########################################
function execute_step() {
  local log_path="/tmp/tbar-build.log"
  local delay=0.05
  local frames=("⠋" "⠙" "⠹" "⠸" "⠼" "⠴" "⠦" "⠧" "⠇" "⠏")
  local check_symbol="✓"
  local x_symbol="⨯"
  local index=0
  local framesCount="${#frames[@]}"

  eval $1 >${log_path} 2>&1 & pid=$!
  
  tput civis # Hide the cursor

  while [ "$(ps a | awk '{print $1}' | grep ${pid})" ]; do
    echo -ne " ${YELLOW}${frames[$index]} $2${NC}"
    index=$(( (${index}+1) % ${framesCount} ))
    for i in {1..90}; do echo -en "\b"; done
    sleep ${delay}
  done

  wait $!

  if [[ "$?" -eq "0" ]]; then
    echo -ne " ${GREEN}${check_symbol} $2\n"
  else
    echo -ne " ${RED}${x_symbol} $2\n"
    cat ${log_path}
    tput cnorm
    exit 1
  fi
  
  tput cnorm # Restore the cursor
}

print_logo() {
  echo -e "${GREEN}   __${NC}  __           "
  echo -e "${GREEN}  / /_${NC}/ /  ___ _____"
  echo -e "${GREEN} / __/${NC} _ \/ _ \`/ __/"
  echo -e "${GREEN} \__/${NC}_.__/\_,_/_/   "
  echo -e "${GREEN} ....${NC}..............."
}

check_if_cargo_exists() {
  if ! command -v cargo &> /dev/null; then
    echo "cargo could not be found"
    exit
  fi
}

apply_config() {
  local config_file="./configs/${ARGS_CONFIG}.rs";

  if ! test -f "${config_file}"; then
    echo -e "  ${RED}${config_file} could not be found"
    exit 1
  fi

  cp "${config_file}" "./src/config.rs"
}


main() {
  parse_args "$@"
  print_logo
  check_if_cargo_exists
  if [ ! -z "${ARGS_CONFIG}" ]; then execute_step "apply_config" "selecting config"; fi
  
  execute_step "cargo build" "building"
  cp ./target/wasm32-wasi/debug/tbar.wasm ./
  execute_step "" "done" 
}

main "$@"
