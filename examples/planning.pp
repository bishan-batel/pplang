

function neg() => -x

function main() {
}


function linear_search(array: *i32, size: usize, target: i32) -> i32 {

    let a_cpy: Pointer[i32];

    for (var i = 0, i < 10, i += 1) {
        if array[i] == target  {
            return i
        }
    }

    return -1
}