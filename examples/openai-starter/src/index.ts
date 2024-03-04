import * as OpenAI from 'openai';

const configuration = new OpenAI.Configuration({
  apiKey: process.env.OPENAI_API_KEY,
});

export const openAIClient = new OpenAI.OpenAIApi(configuration);
