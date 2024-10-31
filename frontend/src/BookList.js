import React, { useState } from 'react';
import axios from 'axios';

const BookList = () => {
    const [books, setBooks] = useState([]);
    const [loading, setLoading] = useState(false);

    const fetchBooks = async () => {
        setLoading(true);
        try {
            const response = await axios.get('http://localhost:3000/books');
            setBooks(response.data.books);
        } catch (error) {
            console.error('Error fetching books:', error);
        } finally {
            setLoading(false);
        }
    };

    return (
        <div>
            <h1>Book List</h1>
            <button onClick={fetchBooks} disabled={loading}>
                {loading ? 'Loading...' : 'Fetch Books'}
            </button>
            <ul>
                {books.map((book, index) => (
                    <li key={index}>
                        <strong>{book.title}</strong> by {book.author.first_name} {book.author.last_name} - Genre: {book.genre}
                    </li>
                ))}
            </ul>
        </div>
    );
};

export default BookList;
