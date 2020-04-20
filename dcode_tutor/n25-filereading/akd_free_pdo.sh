#!/bin/bash

POS=-p0
# clear RxPDO
ethercat $POS --type uint8 download 0x1C12 0 0 # clear RxPDO counter
ethercat $POS --type uint8 download 0x1600 0 0 # clear RxPDO0
ethercat $POS --type uint8 download 0x1601 0 0 # clear RxPDO1
ethercat $POS --type uint8 download 0x1602 0 0 # clear RxPDO2
ethercat $POS --type uint8 download 0x1603 0 0 # clear RxPDO3

# define RxPdo
ethercat $POS --type uint32 download 0x1600 1 0x60400010 # control word
ethercat $POS --type uint32 download 0x1600 2 0x60C10120 # 1st interp target pos
ethercat $POS --type uint8 download 0x1600 0 2 # number of var in this PDO
ethercat $POS --type uint16 download 0x1C12 1 0x1600 # list RxPdo
ethercat $POS --type uint8  download 0x1C12 0 1 # number of RxPdo


# clear TxPdo
ethercat $POS --type uint8  download 0x1C13 0 0 # clear TxPDO counter
ethercat $POS --type uint8 download 0x1A00 0 0  # clear TxPDO0
ethercat $POS --type uint8 download 0x1A01 0 0  # clear TxPDO1
ethercat $POS --type uint8 download 0x1A02 0 0  # clear TxPDO2
ethercat $POS --type uint8 download 0x1A03 0 0  # clear TxPDO3

# define TxPdo
ethercat $POS --type uint32 download 0x1A00 1 0x60410010 # status word
ethercat $POS --type uint32 download 0x1A00 2 0x34700410 # analog input
ethercat $POS --type uint32 download 0x1A00 3 0x60FD0020 # digital inputs
ethercat $POS --type uint8 download 0x1A00 0 3 # number of var in this PDO

ethercat $POS --type uint32 download 0x1A01 1 0x606c0020 # act velocity
ethercat $POS --type uint32 download 0x1A01 2 0x60630020 # act position
ethercat $POS --type uint8 download 0x1A01 0 2 # number of var in this PDO

ethercat $POS --type uint16  download 0x1C13 1 0x1A00 # list TxPdo
ethercat $POS --type uint16  download 0x1C13 2 0x1A01 # list TxPdo
ethercat $POS --type uint8  download 0x1C13 0 2 # number of TxPdo

ethercat rescan
sleep 5
ethercat cstruct $POS
