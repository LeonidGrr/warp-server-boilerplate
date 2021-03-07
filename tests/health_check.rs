// curl --request POST --data 'name=le%20guin&email=ursula_le_guin%40gmail.com&password=1234qwer' 127.0.0.1:8000/register --verbose
// curl --request POST --data 'name=le%20guin&password=123456qw' 127.0.0.1:8000/login --verbose
// curl -v -d "" --cookie 'session=880e182d1cf248d7a8e779ce0250164d' 127.0.0.1:8000/logout
// curl -v -d "" --cookie 'session=880e182d1cf248d7a8e779ce0250164d' 127.0.0.1:8000/test
