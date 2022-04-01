import React, { useState } from 'react';
import './App.css';
import useAxios, {configure} from 'axios-hooks'
import Axios from 'axios'

const axios = Axios.create({
	baseURL: 'http://localhost:8000',
	withCredentials: true
})

configure({ axios })

function App() {
	const [key, setKey] = useState('');
	const [value, setValue] = useState('');
	const [, send] = useAxios({url: 'set_cookies', method: 'post'}, {manual: true})

	const onClick = () => {
		send({params: {
			key, value
		}})
	}

  return (
    <div className="App">
			<input value={key} onChange={e => setKey(e.target.value)}/>
			<input value={value} onChange={e => setValue(e.target.value)}/>
			<button onClick={onClick}>set {key} = {value}</button>
    </div>
  );
}

export default App;
