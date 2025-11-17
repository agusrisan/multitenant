interface PasswordStrengthProps {
  password: string
}

type StrengthLevel = {
  level: number
  label: 'weak' | 'medium' | 'strong'
}

export default function PasswordStrength({ password }: PasswordStrengthProps) {
  const strength = calculateStrength(password)

  const colors = {
    weak: 'bg-red-500',
    medium: 'bg-yellow-500',
    strong: 'bg-green-500',
  }

  return (
    <div className="space-y-1">
      <div className="flex gap-1">
        {[1, 2, 3].map((level) => (
          <div
            key={level}
            className={`h-1 flex-1 rounded ${
              level <= strength.level ? colors[strength.label] : 'bg-gray-200'
            }`}
          />
        ))}
      </div>
      <p className="text-xs text-gray-600">
        Strength: {strength.label}
      </p>
    </div>
  )
}

function calculateStrength(password: string): StrengthLevel {
  if (password.length < 8) return { level: 1, label: 'weak' }
  if (password.length < 12) return { level: 2, label: 'medium' }
  return { level: 3, label: 'strong' }
  // Add more sophisticated logic (uppercase, numbers, symbols)
}

// Named export for backward compatibility
export { PasswordStrength }
