static func foo():
    print(self)

    func bar():
        print(self) # should be okay

        static func baz():
            print(self) # should NOT be okay
