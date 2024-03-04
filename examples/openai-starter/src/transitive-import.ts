import { openAIClient } from "./index";

async function main() {
  try {
    const engineId = "text-davinci-003"; // Engine ID
    const prompt = "Hello world"; // Your prompt

    const response = await openAIClient.createCompletion(engineId, { prompt: prompt });

    console.log((response.data.choices ?? [])[0].text?.trim());
  } catch (error) {
    console.error("Error calling OpenAI API:", error);
  }
}

main();
