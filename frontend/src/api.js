import axios from 'axios';

export default async function sendSub(pincode, age, token) {
    try {
        const response = await axios.post('/api/v1/subscribe', {
            pincode: pincode,
            age: age,
            token: token,
        })
    } catch (error) {
        throw 'Something went wrong: Error sending API Call';
    }
}
