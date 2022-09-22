# // Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
# // SPDX-License-Identifier: MIT-0

#!/bin/sh

# Assign an IP address to local loopback 
ip addr add 127.0.0.1/32 dev lo

ip link set dev lo up

# Run the vsock-to-ip proxy in background and start enclave server 
/app/vsock-to-ip -v "16:8000" -i "127.0.0.1:32" &
/app/enclave-server  
