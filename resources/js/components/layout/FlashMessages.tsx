import { usePage } from '@inertiajs/react'
import { Alert, AlertDescription } from '@/components/ui/alert'
import { PageProps } from '@/types'

export function FlashMessages() {
  const { flash } = usePage<PageProps>().props

  if (!flash) return null

  return (
    <div className="space-y-2">
      {flash.success && (
        <Alert>
          <AlertDescription>{flash.success}</AlertDescription>
        </Alert>
      )}
      {flash.error && (
        <Alert variant="destructive">
          <AlertDescription>{flash.error}</AlertDescription>
        </Alert>
      )}
    </div>
  )
}
