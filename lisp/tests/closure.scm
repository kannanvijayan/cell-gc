((lambda (make-add)
   ((lambda (add)
      (assert (eq? 8 (add 3))))
    (make-add 5)))
 (lambda (x)
   (lambda (y)
     (+ x y))))
