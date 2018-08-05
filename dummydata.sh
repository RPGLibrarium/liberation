#!/bin/bash

DSA40=$(curl -v --silent 'http://127.0.0.1:8080/v1/rpgsystems' -H "Content-Type: application/json" -X POST --data-raw '{"rpgsystem":{"name":"Das Schwarze Auge 4.0"}}' 2>&1 | grep -Fi location | sed 's/.*\/\([0-9]\+\)/\1/')

DSAZauberei=$(curl -v 'http://127.0.0.1:8080/v1/titles' -H "Content-Type: application/json" -X POST --data-raw "{\"title\":{\"name\":\"Zauberei\", \"system\": $DSA40, \"language\": \"de_DE\", \"publisher\": \"Ulisses\",  \"year\": 2000, \"coverimage\": \"\"}}" 2>&1 | grep -Fi location | sed 's/.*\/\([0-9]\+\)/\1/')

DSASuR=$(curl -v 'http://127.0.0.1:8080/v1/titles' -H "Content-Type: application/json" -X POST --data-raw "{\"title\":{\"name\":\"Schwerter und Helden\", \"system\": $DSA40, \"language\": \"de_DE\", \"publisher\": \"Ulisses\",  \"year\": 2000, \"coverimage\": \"\"}}" 2>&1 | grep -Fi location | sed 's/.*\/\([0-9]\+\)/\1/')

DSA41=$(curl -v --silent 'http://127.0.0.1:8080/v1/rpgsystems' -H "Content-Type: application/json" -X POST --data-raw '{"rpgsystem":{"name":"Das Schwarze Auge 4.1"}}' 2>&1 | grep -Fi location | sed 's/.*\/\([0-9]\+\)/\1/')

Basisregelwerk=$(curl -v 'http://127.0.0.1:8080/v1/titles' -H "Content-Type: application/json" -X POST --data-raw "{\"title\":{\"name\":\"Basisregelwerk\", \"system\": $DSA41, \"language\": \"de_DE\", \"publisher\": \"Ulisses\",  \"year\": 2006, \"coverimage\": \"\"}}" 2>&1 | grep -Fi location | sed 's/.*\/\([0-9]\+\)/\1/')

WegeDerHelden=$(curl -v 'http://127.0.0.1:8080/v1/titles' -H "Content-Type: application/json" -X POST --data-raw "\"{\"title\":{\"name\":\"Wege der Helden\", \"system\": $DSA41, \"language\": \"de_DE\", \"publisher\": \"Ulisses\",  \"year\": 2007, \"coverimage\": \"\"}}" 2>&1 | grep -Fi location | sed 's/.*\/\([0-9]\+\)/\1/')

WegeDerZauberei=$(curl -v 'http://127.0.0.1:8080/v1/titles' -H "Content-Type: application/json" -X POST --data-raw "{\"title\":{\"name\":\"Wege der Zauberei\", \"system\": $DSA41, \"language\": \"de_DE\", \"publisher\": \"Ulisses\",  \"year\": 2008, \"coverimage\": \"\"}}" 2>&1 | grep -Fi location | sed 's/.*\/\([0-9]\+\)/\1/')

DSA5=$(curl -v --silent 'http://127.0.0.1:8080/v1/rpgsystems' -H "Content-Type: application/json" -X POST --data-raw '{"rpgsystem":{"name":"Das Schwarze Auge 5"}}' 2>&1 | grep -Fi location | sed 's/.*\/\([0-9]\+\)/\1/')

DSA5Grundregelwerk=$(curl -v 'http://127.0.0.1:8080/v1/titles' -H "Content-Type: application/json" -X POST --data-raw "{\"title\":{\"name\":\"Das Schwarze Auge Grundregelwerk\", \"system\": $DSA5, \"language\": \"de_DE\", \"publisher\": \"Ulisses\",  \"year\": 2014, \"coverimage\": \"\"}}" 2>&1 | grep -Fi location | sed 's/.*\/\([0-9]\+\)/\1/')

AventurischerAlamanach=$(curl -v 'http://127.0.0.1:8080/v1/titles' -H "Content-Type: application/json" -X POST --data-raw "{\"title\":{\"name\":\"Aventurischer Almanach\", \"system\": $DSA5, \"language\": \"de_DE\", \"publisher\": \"Ulisses\",  \"year\": 2015, \"coverimage\": \"\"}}" 2>&1 | grep -Fi location | sed 's/.*\/\([0-9]\+\)/\1/')

AventurischeMagie=$(curl -v 'http://127.0.0.1:8080/v1/titles' -H "Content-Type: application/json" -X POST --data-raw "{\"title\":{\"name\":\"Aventurische Magie\", \"system\": $DSA5, \"language\": \"de_DE\", \"publisher\": \"Ulisses\",  \"year\": 2015, \"coverimage\": \"\"}}" 2>&1 | grep -Fi location | sed 's/.*\/\([0-9]\+\)/\1/')

SR5=$(curl -v --silent 'http://127.0.0.1:8080/v1/rpgsystems' -H "Content-Type: application/json" -X POST --data-raw '{"rpgsystem":{"name":"Shadowrun 5"}}' 2>&1 | grep -Fi location | sed 's/.*\/\([0-9]\+\)/\1/')

SR5Grundregelwerk=$(curl -v 'http://127.0.0.1:8080/v1/titles' -H "Content-Type: application/json" -X POST --data-raw "{\"title\":{\"name\":\"SR5 Grundregelwerk\", \"system\": $SR5, \"language\": \"de_DE\", \"publisher\": \"Pegasus Spiele\",  \"year\": 2014, \"coverimage\": \"\"}}" 2>&1 | grep -Fi location | sed 's/.*\/\([0-9]\+\)/\1/')

Kreuzfeuer=$(curl -v 'http://127.0.0.1:8080/v1/titles' -H "Content-Type: application/json" -X POST --data-raw "{\"title\":{\"name\":\"Kreuzfeuer\", \"system\": $SR5, \"language\": \"de_DE\", \"publisher\": \"Pegasus Spiele\",  \"year\": 2015, \"coverimage\": \"\"}}" 2>&1 | grep -Fi location | sed 's/.*\/\([0-9]\+\)/\1/')

Schattenhandbuch1=$(curl -v 'http://127.0.0.1:8080/v1/titles' -H "Content-Type: application/json" -X POST --data-raw "{\"title\":{\"name\":\"Schattenhandbuch 1\", \"system\": $SR5, \"language\": \"de_DE\", \"publisher\": \"Pegasus Spiele\",  \"year\": 2016, \"coverimage\": \"\"}}" 2>&1 | grep -Fi location | sed 's/.*\/\([0-9]\+\)/\1/')

Schattenhandbuch2=$(curl -v 'http://127.0.0.1:8080/v1/titles' -H "Content-Type: application/json" -X POST --data-raw "{\"title\":{\"name\":\"Schattenhandbuch 2\", \"system\": $SR5, \"language\": \"de_DE\", \"publisher\": \"Pegasus Spiele\",  \"year\": 2017, \"coverimage\": \"\"}}" 2>&1 | grep -Fi location | sed 's/.*\/\([0-9]\+\)/\1/')

Schattenhandbuch3=$(curl -v 'http://127.0.0.1:8080/v1/titles' -H "Content-Type: application/json" -X POST --data-raw "{\"title\":{\"name\":\"Schattenhandbuch 3\", \"system\": $SR5, \"language\": \"de_DE\", \"publisher\": \"Pegasus Spiele\",  \"year\": 2018, \"coverimage\": \"\"}}" 2>&1 | grep -Fi location | sed 's/.*\/\([0-9]\+\)/\1/')
