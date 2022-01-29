echo "Starting up the following containers: \n * Headless Selenium - Port 4444 \n * MySQL "
echo "Starting Headless Selenium container."
sudo docker run --rm -d -p 4444:4444 --name selenium-server -v /dev/shm:/dev/shm --shm-size="2g" selenium/standalone-chrome:4.1.1-20211217
echo "Selenium Docker started!"
echo "Starting MySQL container."
sudo docker start mysql_docker
source_fp=/home/workspace-brandon/Documents/Credentials/Credentials
temp_fp=/dev/shm/Credentials.txt
gpg -o $temp_fp -d $source_fp ; read -a arr <<< $(grep -m 1 mysql_docker $temp_fp) ; rm $temp_fp
sudo docker exec -it mysql_docker bash -c "mysql -u root --password=${arr[1]}" ; exit
echo "MySql docker started!"
sudo docker container ls -a