# Jak skompilować
- Ściągnąć Rustupem najnowszego Rusta i Cargo.
- Odpalić skrypt `build.sh`: 
```
chmod +x ./build.sh
./build.sh
```
# Jak uruchomić po zbudowaniu w miejscu
```
cd release
DATABASE_URL='postgres://{username}:{password}@{hostname}/{database}' PORT='{numer portu}' ./book-renderer
```
Klamr nie piszemy. Po uruchomieniu apka będzie nasłuchiwać na podanym numerze portu, wtedy w przeglądarce wchodzimy na adres `127.0.0.1:{numer portu}/`.
Jeśli zmienne środowiskowe DATABASE_URL i PORT są już ustawione, nie ma potrzeby wpisywania ich przy uruchamianiu.
Zrzut ekranu:
![Zrzut ekranu](screenshot.png)
# Przykład użycia
```
DATABASE_URL='postgres://ja:mojehasło@serwer.com/baza_danych' PORT=8080 book-renderer
```