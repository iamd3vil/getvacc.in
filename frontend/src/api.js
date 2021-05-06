import axios from 'axios';

export default async function sendSub(pincode, age, token) {
    try {
        const response = await axios.post('/api/v1/subscribe', {
            pincode: pincode,
            age_limit: age,
            token: token,
        })

        console.log('response: ', response)
    } catch (error) {
        console.log('Error sending API call: ', error)
    }
}