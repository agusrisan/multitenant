import { PageProps as InertiaPageProps } from '@inertiajs/core'
import { PageProps as AppPageProps } from './index'

declare module '@inertiajs/core' {
  interface PageProps extends InertiaPageProps, AppPageProps {}
}
