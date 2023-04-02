#lang racket

; setup variables
(define n 0)
(define current 0)

; the main parsing loop
(define lines (file->lines "aoc-2022/day1/input.txt"))
(for ([line lines])
  (begin
    (define len (length (string->list line)))
    (if (not (eq? len 0))
        (set! current (+ current (string->number line)))
        (begin
          (set! n (max n current))
          (set! current 0)))))

; make sure to flush any final trailing data
(set! n (max n current))

; print the max value
(displayln n)
