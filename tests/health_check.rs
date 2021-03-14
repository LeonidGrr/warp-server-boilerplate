// curl -v -X POST -d 'name=le%20guin&email=ursula_le_guin%40gmail.com&password=1234qwer' 127.0.0.1:8000/register
// curl -v -X POST -d 'name=le%20guin&password=123456qw' 127.0.0.1:8000/login
// curl -v -d '' -b 'session=133d6adc0c0848308af328d623bdbda9' 127.0.0.1:8000/test
// curl -v -d '' -b 'session=133d6adc0c0848308af328d623bdbda9' 127.0.0.1:8000/api/test
// curl -v -d '' -b 'session=fake' 127.0.0.1:8000/api/test
// curl -v -d '' -b 'session=133d6adc0c0848308af328d623bdbda9' 127.0.0.1:8000/logout

