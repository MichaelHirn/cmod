import { OpenAIApi } from "openai";

const openai = new OpenAIApi({ apiKey: process.env.OPENAI_API_KEY } as any);

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
