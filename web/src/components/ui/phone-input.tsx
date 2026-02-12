"use client"

import * as React from "react"
import { Check, ChevronsUpDown } from "lucide-react"
import {
  getCountries,
  getCountryCallingCode,
  isPossiblePhoneNumber,
  isValidPhoneNumber,
  parsePhoneNumberFromString,
  type CountryCode,
} from "libphonenumber-js/min"

import { cn } from "@/lib/utils"
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command"
import {
  InputGroup,
  InputGroupAddon,
  InputGroupButton,
  InputGroupInput,
} from "@/components/ui/input-group"
import { Popover, PopoverContent, PopoverTrigger } from "@/components/ui/popover"

type CountryOption = {
  code: CountryCode
  name: string
  dialCode: string
  searchValue: string
}

type PhoneValidationResult = {
  country: CountryCode
  isValid: boolean
  e164: string | null
  error: string | null
}

interface PhoneInputProps
  extends Omit<
    React.ComponentProps<"input">,
    "type" | "value" | "defaultValue" | "onChange"
  > {
  value?: string
  defaultValue?: string
  onChange?: (event: React.ChangeEvent<HTMLInputElement>) => void
  onValueChange?: (value: string) => void
  onValidationChange?: (result: PhoneValidationResult) => void
  country?: CountryCode
  defaultCountry?: CountryCode
  onCountryChange?: (country: CountryCode) => void
  containerClassName?: string
  messageClassName?: string
  showValidationMessage?: boolean
}

const DEFAULT_COUNTRY: CountryCode = "US"

const regionNames =
  typeof Intl !== "undefined" && typeof Intl.DisplayNames === "function"
    ? new Intl.DisplayNames(["en"], { type: "region" })
    : null

const countryOptions: CountryOption[] = getCountries()
  .map((code) => {
    const name = regionNames?.of(code) ?? code
    const dialCode = `+${getCountryCallingCode(code)}`

    return {
      code,
      name,
      dialCode,
      searchValue: `${name} ${code} ${dialCode}`,
    }
  })
  .sort((a, b) => a.name.localeCompare(b.name))

const countryOptionByCode = new Map(
  countryOptions.map((option) => [option.code, option])
)

function validatePhoneNumber(
  value: string,
  country: CountryCode
): PhoneValidationResult {
  const input = value.trim()

  if (!input) {
    return {
      country,
      isValid: false,
      e164: null,
      error: null,
    }
  }

  const hasCountryCode = input.startsWith("+")
  const isPossible = hasCountryCode
    ? isPossiblePhoneNumber(input)
    : isPossiblePhoneNumber(input, country)

  if (!isPossible) {
    return {
      country,
      isValid: false,
      e164: null,
      error: "Phone number length is invalid for this country.",
    }
  }

  const parsed = hasCountryCode
    ? parsePhoneNumberFromString(input)
    : parsePhoneNumberFromString(input, country)

  if (!parsed || !isValidPhoneNumber(parsed.number)) {
    return {
      country,
      isValid: false,
      e164: null,
      error: "Enter a valid phone number.",
    }
  }

  return {
    country: parsed.country ?? country,
    isValid: true,
    e164: parsed.number,
    error: null,
  }
}

function PhoneInput({
  value,
  defaultValue = "",
  onChange,
  onValueChange,
  onValidationChange,
  country,
  defaultCountry = DEFAULT_COUNTRY,
  onCountryChange,
  className,
  containerClassName,
  messageClassName,
  showValidationMessage = true,
  placeholder = "Enter phone number",
  onBlur,
  disabled,
  ...props
}: PhoneInputProps) {
  const [open, setOpen] = React.useState(false)
  const [touched, setTouched] = React.useState(false)
  const [internalValue, setInternalValue] = React.useState(defaultValue)
  const [internalCountry, setInternalCountry] =
    React.useState<CountryCode>(defaultCountry)

  const selectedCountry = country ?? internalCountry
  const inputValue = value ?? internalValue

  const selectedOption =
    countryOptionByCode.get(selectedCountry) ??
    countryOptionByCode.get(DEFAULT_COUNTRY)!

  const validation = React.useMemo(
    () => validatePhoneNumber(inputValue, selectedCountry),
    [inputValue, selectedCountry]
  )

  React.useEffect(() => {
    onValidationChange?.(validation)
  }, [onValidationChange, validation])

  const isAriaInvalid =
    props["aria-invalid"] === true || props["aria-invalid"] === "true"
  const showError = Boolean(
    validation.error && inputValue.trim().length > 0 && touched
  )

  const handleChange = React.useCallback(
    (event: React.ChangeEvent<HTMLInputElement>) => {
      setTouched(true)
      if (value === undefined) {
        setInternalValue(event.target.value)
      }
      onValueChange?.(event.target.value)
      onChange?.(event)
    },
    [onChange, onValueChange, value]
  )

  const handleCountrySelect = React.useCallback(
    (nextCountry: CountryCode) => {
      setOpen(false)
      if (country === undefined) {
        setInternalCountry(nextCountry)
      }
      onCountryChange?.(nextCountry)
    },
    [country, onCountryChange]
  )

  return (
    <div data-slot="phone-input" className={cn("space-y-1", containerClassName)}>
      <InputGroup>
        <InputGroupAddon className="pr-1">
          <Popover open={open} onOpenChange={setOpen}>
            <PopoverTrigger asChild>
              <InputGroupButton
                variant="ghost"
                size="sm"
                role="combobox"
                aria-label="Select country"
                aria-expanded={open}
                className="h-7 gap-1 rounded-sm px-2 text-xs"
                disabled={disabled}
              >
                <span className="font-medium">{selectedOption.code}</span>
                <span className="text-muted-foreground">
                  {selectedOption.dialCode}
                </span>
                <ChevronsUpDown className="size-3 opacity-60" />
              </InputGroupButton>
            </PopoverTrigger>
            <PopoverContent className="w-[320px] p-0" align="start">
              <Command>
                <CommandInput placeholder="Search country or code..." />
                <CommandList>
                  <CommandEmpty>No country found.</CommandEmpty>
                  <CommandGroup>
                    {countryOptions.map((option) => (
                      <CommandItem
                        key={option.code}
                        value={option.searchValue}
                        onSelect={() => handleCountrySelect(option.code)}
                      >
                        <Check
                          className={cn(
                            "size-4",
                            option.code === selectedCountry
                              ? "opacity-100"
                              : "opacity-0"
                          )}
                        />
                        <span className="flex flex-1 items-center justify-between gap-2">
                          <span>{option.name}</span>
                          <span className="text-muted-foreground text-xs">
                            {option.dialCode}
                          </span>
                        </span>
                      </CommandItem>
                    ))}
                  </CommandGroup>
                </CommandList>
              </Command>
            </PopoverContent>
          </Popover>
        </InputGroupAddon>

        <InputGroupInput
          {...props}
          type="tel"
          inputMode="tel"
          autoComplete="tel-national"
          value={inputValue}
          onChange={handleChange}
          onBlur={(event) => {
            setTouched(true)
            onBlur?.(event)
          }}
          placeholder={placeholder}
          disabled={disabled}
          aria-invalid={isAriaInvalid || showError}
          className={className}
        />
      </InputGroup>

      {showValidationMessage && showError ? (
        <p
          data-slot="phone-input-message"
          className={cn("text-destructive text-sm", messageClassName)}
        >
          {validation.error}
        </p>
      ) : null}
    </div>
  )
}

export { PhoneInput, type PhoneInputProps, type PhoneValidationResult }
