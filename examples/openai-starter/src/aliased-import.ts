import { Configuration, OpenAIApi as OpenAIClient } from "openai";

const configuration = new Configuration({
  apiKey: process.env.OPENAI_API_KEY,
});

const openai = new OpenAIClient(configuration);

async function main() {
  try {
    const engineId = "text-davinci-003"; // Engine ID
    const prompt = "Hello world"; // Your prompt

    const response = await openai.createCompletion(engineId, { prompt: prompt });

    console.log((response.data.choices ?? [])[0].text?.trim());
  } catch (error) {
    console.error("Error calling OpenAI API:", error);
  }
}

main();
