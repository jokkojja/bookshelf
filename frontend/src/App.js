import React, { useEffect, useState } from 'react';
import { fetchAuthors, fetchGenres, fetchBooks } from './api';

function App() {
  const [authors, setAuthors] = useState([]);
  const [genres, setGenres] = useState([]);
  const [books, setBooks] = useState([]);

  useEffect(() => {
    fetchAuthors().then(setAuthors);
    fetchGenres().then(setGenres);
    fetchBooks().then(setBooks);
  }, []);

  return (
    <div>
      <h1>Library</h1>
      <h2>Books</h2>
      <ul>
        {books.map((book, index) => (
          <li key={index}>{book.title}</li>
        ))}
      </ul>

      <h2>Authors</h2>
      <ul>
        {authors.map((author, index) => (
          <li key={index}>{author.first_name} {author.last_name}</li>
        ))}
      </ul>

      <h2>Genres</h2>
      <ul>
        {genres.map((genre, index) => (
          <li key={index}>{genre}</li>
        ))}
      </ul>
    </div>
  );
}

export default App;