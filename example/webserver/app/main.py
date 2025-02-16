from typing import Annotated
from rust_idl import hello

from typing import Union
from fastapi import FastAPI, Response, Header

app = FastAPI()

@app.get("/hello/{name}")
def greet(name: str, accept: Annotated[str, Header()] = None):
    greeting = hello.Hello(name)

    if accept == "application/msgpack":
        response = Response(content=greeting.to_msgpack())
        response.headers["Content-Type"] = "application/msgpack"
    else:
        response = Response(content=greeting.to_json())
        response.headers["Content-Type"] = "application/json"

    response.headers["Access-Control-Allow-Origin"] = "*"
    response.headers["Access-Control-Allow-Methods"] = "GET, POST, OPTIONS"
    response.headers["Access-Control-Allow-Headers"] = "Content-Type, Accept"

    return response