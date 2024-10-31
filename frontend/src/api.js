// frontend/src/api.js

const API_URL = "http://localhost:3000";

export async function fetchAuthors() {
    const response = await fetch(`${API_URL}/authors`);
    return response.json();
}

export async function fetchGenres() {
    const response = await fetch(`${API_URL}/genres`);
    return response.json();
}

export async function fetchBooks() {
    const response = await fetch(`${API_URL}/books`);
    return response.json();
}