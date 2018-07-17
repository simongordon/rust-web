.RECIPEPREFIX = >

all: compile run

install:
> npm install

compile:
> wargo build
> npm run compile

run:
> npm run serve
