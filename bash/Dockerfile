FROM ubuntu:20.04

RUN /bin/bash -c "apt-get update && \
                  apt-get install -y curl unzip sudo iputils-ping && \
                  curl 'https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip' -o 'awscliv2.zip' && \
                  unzip awscliv2.zip && \
                  rm awscliv2.zip && \
                  sudo ./aws/install"

WORKDIR project
COPY . .
RUN /bin/sh -c "chmod +x *.sh"

# Build: 
# sudo docker build -t aws-s3-test:bash .
# Run: 
# - [Allocate a tty for the interactive container process] 
#   sudo docker run --rm --network aws-s3-sample_custom-network -it aws-s3-test:bash bash
# - [Use bash script once trough docker container] 
#   sudo docker run --rm --network aws-s3-sample_custom-network \
#   aws-s3-test:bash /bin/bash -c \
#   "./upload-s3-object.sh && \
#    ./download-s3-object.sh && \
#    cat sample-download.txt"