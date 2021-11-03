#!/usr/bin/env python3
"""websocket cmd client for actix/websocket-tcp-chat example."""
import argparse
import asyncio
import signal
import sys
import json

import aiohttp

queue = asyncio.Queue()


async def start_client(url, loop):
    print("coś?")
    ws = await aiohttp.ClientSession().ws_connect(url, autoclose=False, autoping=False)
    print("cokolwiek?")
    print(ws)

    def stdin_callback():
        line = sys.stdin.buffer.readline().decode('utf-8')
        if not line:
            loop.stop()
        else:
            x = line.split(" ", 2)
            if x[0] == "create_game":
                data = {
                    "room": x[1].strip(),
                    "players": [2, 3],
                    "host": 1,
                }
            elif x[0] == "validate_answer":
                data = {
                    "room": x[1].strip(),
                    "game_id": 1,
                    "is_answer_valid": x[2].strip(),
                }
            elif x[0] == "choose_answerer":
                data = {
                    "room": x[1].strip(),
                    "game_id": 1,
                    "new_answerer_id": int(x[2].strip()),
                }
            else:
                data = {
                    "room": x[1].strip(),
                    "game_id": 1,
                    "contents": x[2].strip() if len(x) == 3 else None,
                }
            # Queue.put is a coroutine, so you can't call it directly.
            asyncio.ensure_future(queue.put(ws.send_str("{} {}".format(x[0].strip(), json.dumps(data)))))

    loop.add_reader(sys.stdin, stdin_callback)

    async def dispatch():
        while True:
            msg = await ws.receive()
            if msg.type == aiohttp.WSMsgType.TEXT:
                print('Text: ', msg.data.strip())
            elif msg.type == aiohttp.WSMsgType.BINARY:
                print('Binary: ', msg.data)
            elif msg.type == aiohttp.WSMsgType.PING:
                await ws.pong()
            elif msg.type == aiohttp.WSMsgType.PONG:
                print('Pong received')
            else:
                if msg.type == aiohttp.WSMsgType.CLOSE:
                    await ws.close()
                elif msg.type == aiohttp.WSMsgType.ERROR:
                    print('Error during receive %s' % ws.exception())
                elif msg.type == aiohttp.WSMsgType.CLOSED:
                    pass
                break

    await dispatch()


async def tick():
    while True:
        await (await queue.get())


async def main(url, loop):
    task1 = loop.create_task(start_client(url, loop))
    task2 = loop.create_task(tick())
    await asyncio.wait({task1, task2})


ARGS = argparse.ArgumentParser(
    description="websocket console client for OOT game.")
ARGS.add_argument(
    '--host', action="store", dest='host',
    default='oot-back.local.net', help='Host name')
ARGS.add_argument(
    '--port', action="store", dest='port',
    default=8888, type=int, help='Port number')

if __name__ == '__main__':
    args = ARGS.parse_args()
    if ':' in args.host:
        args.host, port = args.host.split(':', 1)
        args.port = int(port)

    name = input('Please enter your name: ')
    url = 'http://{}:{}/ws/{}'.format(args.host, args.port, name)
    print(url)

    loop = asyncio.get_event_loop()
    loop.add_signal_handler(signal.SIGINT, loop.stop)
    asyncio.Task(main(url, loop))
    loop.run_forever()
