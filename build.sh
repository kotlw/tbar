#!/bin/bash

declare -xr NC="\033[0m"
declare -xr RED="\033[31m"
declare -xr GREEN="\033[32m"
declare -xr YELLOW="\033[93m"

declare -xr GIT_URL="https://github.com/kotlw/tbar"
declare -xr DIR="$HOME/.config/zellij/plugins/tbar"

declare ARGS_CONFIG="compact-bar" # default config
declare ARGS_DOWNLOAD=0

parse_args() {
  while [ "$#" -gt 0 ]; do
    case "$1" in
      -c | --config) shift; ARGS_CONFIG=$1 ;;
      -d | --download) ARGS_DOWNLOAD=1 ;;
    esac
    shift
  done
}

#############################################
# Run the command passed as 1st argument and 
# shows the spinner until this is done
# Arguments:
#   $1 the command to run
#   $2 the title to show next to the spinner
#############################################
function execute_step() {
  local log_path="/tmp/tbar-build.log"
  local delay=0.05
  local frames=("⠋" "⠙" "⠹" "⠸" "⠼" "⠴" "⠦" "⠧" "⠇" "⠏")
  local check_symbol="✓"
  local x_symbol="⨯"
  local index=0
  local framesCount="${#frames[@]}"

  $($1 >${log_path} 2>&1) & pid=$!
  
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
    cat "${log_path}" | while read line; do echo "  ${line}"; done
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

print_footer() {
  echo -e "${NC} ..................."
  echo -e " Path to the plugin binary: file:~/.config/zellij/plugins/tbar.wasm"
}

check_dependencies() {
  if ! command -v cargo &> /dev/null; then
    echo -ne "  ${RED}cargo could not be found\n"
    exit 1
  fi

  if ! command -v git &> /dev/null; then
    echo -ne "  ${RED}git could not be found\n"
    exit 1
  fi
}

download() {
  mkdir -p "${DIR}"
  git clone "${GIT_URL}" "${DIR}"
}

apply_config() {
  local config_file="${DIR}/configs/${ARGS_CONFIG}.rs";

  if ! test -f "${config_file}"; then
    echo -e "${RED}${config_file} could not be found"
    exit 1
  fi

  cp "${config_file}" "${DIR}/src/config.rs"
}

move_artifact() {
  cp "${DIR}/target/wasm32-wasi/release/tbar.wasm" "${DIR}/../"
}

main() {
  parse_args "$@"
  print_logo

  execute_step "check_dependencies" "check dependencies"
  [[ "${ARGS_DOWNLOAD}" -eq 1 ]] && execute_step "download" "downloading"
  cd "${DIR}"
  execute_step "apply_config" "applying ${ARGS_CONFIG} config"
  execute_step "cargo build --release" "building"
  execute_step "move_artifact" "moving artifact to the plugin root"
  execute_step "" "done" 
  
  print_footer
}

main "$@"
