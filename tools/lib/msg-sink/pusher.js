// Copyright 2018 BEE project contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

const Pusher = require('pusher');
const readline = require('readline');

// entry point

function run(options) {
  return new Promise((resolve, reject) => {
    if (process.env.BEE_TOOLS_MSG_SINK_PUSHER_APP_ID === undefined) {
      reject(new Error('env.BEE_TOOLS_MSG_SINK_PUSHER_APP_ID is required'));
      return;
    }
    if (process.env.BEE_TOOLS_MSG_SINK_PUSHER_KEY === undefined) {
      reject(new Error('env.BEE_TOOLS_MSG_SINK_PUSHER_KEY is required'));
      return;
    }
    if (process.env.BEE_TOOLS_MSG_SINK_PUSHER_SECRET === undefined) {
      reject(new Error('env.BEE_TOOLS_MSG_SINK_PUSHER_SECRET is required'));
      return;
    }
    if (process.env.BEE_TOOLS_MSG_SINK_PUSHER_CLUSTER === undefined) {
      reject(new Error('env.BEE_TOOLS_MSG_SINK_PUSHER_CLUSTER is required'));
      return;
    }
    if (options.channel === undefined) {
      reject(new Error('channel name is required'));
      return;
    }

    const pusher = new Pusher({
      appId: process.env.BEE_TOOLS_MSG_SINK_PUSHER_APP_ID,
      key: process.env.BEE_TOOLS_MSG_SINK_PUSHER_KEY,
      secret: process.env.BEE_TOOLS_MSG_SINK_PUSHER_SECRET,
      cluster: process.env.BEE_TOOLS_MSG_SINK_PUSHER_CLUSTER,
      encrypted: true
    });

    const rl = readline.createInterface({
      input: process.stdin
    });

    // Pusher seems not to keep the order of messages.  For keeping it, the
    // sequence number is added to data sent to Pusher.
    let seqNo = 0;

    rl.on('line', (line) => {
      try {
        const msg = JSON.parse(line);
        pusher.trigger(options.channel, 'paint', {
          seqNo: seqNo,
          message: msg
        });
        if (msg.type === 'layout.painter.end') {
          seqNo = 0;
        } else {
          ++seqNo;
        }
      } catch (e) {
        reject(e);
      }
    });

    rl.on('close', resolve);
  });
}

// exports

module.exports.run = run;
