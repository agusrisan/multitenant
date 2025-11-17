import axios from 'axios'
import { router } from '@inertiajs/react'

// Configure Axios
axios.defaults.headers.common['X-Requested-With'] = 'XMLHttpRequest'
axios.defaults.withCredentials = true

// CSRF token handling
const token = document.head.querySelector('meta[name="csrf-token"]')
if (token) {
  axios.defaults.headers.common['X-CSRF-Token'] = (token as HTMLMetaElement).content
}

// Inertia progress bar
import NProgress from 'nprogress'
import 'nprogress/nprogress.css'

NProgress.configure({ showSpinner: false })

router.on('start', () => NProgress.start())
router.on('finish', () => NProgress.done())
