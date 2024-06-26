##########################################################################
##    /$$   /$$                     /$$                                 ##
##   | $$  | $$                    |__/                                 ##
##   | $$  | $$  /$$$$$$   /$$$$$$  /$$ /$$$$$$$$  /$$$$$$  /$$$$$$$    ##
##   | $$$$$$$$ /$$__  $$ /$$__  $$| $$|____ /$$/ /$$__  $$| $$__  $$   ##
##   | $$__  $$| $$  \ $$| $$  \__/| $$   /$$$$/ | $$  \ $$| $$  \ $$   ##
##   | $$  | $$| $$  | $$| $$      | $$  /$$__/  | $$  | $$| $$  | $$   ##
##   | $$  | $$|  $$$$$$/| $$      | $$ /$$$$$$$$|  $$$$$$/| $$  | $$   ##
##   |__/  |__/ \______/ |__/      |__/|________/ \______/ |__/  |__/   ##
##########################################################################


#! /bin/bash

##################
# Install Python #
##################

apk add python3

##################
#  Install Git   #
##################

apk add git

##################
# Install GoLang #
##################

# Check GoLang version
go version
