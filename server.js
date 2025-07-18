const express = require('express');
const bodyParser = require('body-parser');
const axios = require('axios');
const { twiml } = require('twilio');

const app = express();
app.use(bodyParser.urlencoded({ extended: false }));

app.post('/sms', async (req, res) => {
  const incomingMsg = req.body.Body;
  const fromNumber = req.body.From;

  const reply = await getLLMReply(incomingMsg, fromNumber);

  const response = new twiml.MessagingResponse();
  response.message(reply);

  res.type('text/xml');
  res.send(response.toString());
});

async function getLLMReply(message, sessionId) {
  // Replace with your actual Retell API call
  return `Hey! I got your message: "${message}" 😊`;
}

app.listen(3000, () => {
  console.log('Server is running on port 3000');
});
