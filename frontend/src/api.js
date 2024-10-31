// frontend/src/api.js

const API_URL = "http://localhost:3000";

export async function fetchBooks() {
    const response = await fetch(`${API_URL}/books`);
    if (!response.ok) {
        throw new Error('Network response was not ok');
    }
    const data = await response.json();
    console.log('Fetched books:', data); // Добавляем лог для проверки
    return data; // Убедитесь, что это массив
}