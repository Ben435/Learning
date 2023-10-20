import http from 'k6/http';
import { check, sleep } from 'k6';

const protocol = 'http'
const host = 'localhost'
const port = '8080'

export const options = {
  stages: [
    { duration: '30s', target: 100 },
    { duration: '1m', target: 100 },
    { duration: '30s', target: 0 },
  ],
  dns: {
    policy: 'preferIPv6'
  }
};

export default function () {
  const res = http.get(`${protocol}://${host}:${port}/`);

  check(res, { 'status was 200': (r) => r.status == 200 });
  sleep(1);
}
