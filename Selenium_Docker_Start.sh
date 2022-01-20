# sudo docker run --rm -d -p 4444:4444 --name selenium-server -e START_XVFB=false -v /dev/shm:/dev/shm --shm-size="2g" selenium/standalone-chrome:4.1.1-20211217
sudo docker run --rm -d -p 4444:4444 --name selenium-server -v /dev/shm:/dev/shm --shm-size="2g" selenium/standalone-chrome:4.1.1-20211217
echo "Selenium Docker started!"
sudo docker ps