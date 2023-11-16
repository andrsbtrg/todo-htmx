source .env

git checkout pi
git pull origin pi
docker build --tag htmx-pi --file Dockerfile .
docker save -o $OUTPUT htmx-pi
rsync -avz $OUTPUT pi:/home/pi/docker-images

ssh pi

cd docker-images

docker load -i htmx.tar 
docker run --network app-network -p 8000:8000 htmx-pi
