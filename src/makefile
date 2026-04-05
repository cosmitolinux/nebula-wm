# n3bu1a shell build script
CC=g++
CFLAGS=-Wall
TARGET=n3bu1a-init
DEST=/usr/local/bin

all: $(TARGET)

$(TARGET): src/n3bu1a-init.cpp
	$(CC) $(CFLAGS) -o $(TARGET) src/n3bu1a-init.cpp

# u need sudo for this part or itll fail lol
install:
	install -m 755 $(TARGET) $(DEST)
	install -m 755 scripts/n3bu1a-ctl.sh $(DEST)/n3bu1a-ctl

clean:
	rm -f $(TARGET)

  # note that this source code was rushed and wont be changed due to questionable reasons -- noted by milocodez
