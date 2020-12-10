use std::collections::HashMap;

use super::super::{challenge_config::ChallengeConfig, challenge_config::ChallengeError};

pub struct Challenge1 {}

impl ChallengeConfig for Challenge1 {
    fn title(&self) -> &str {
        return "1: Multiples of 3 and 5";
    }

    fn description(&self) -> &str {
        return "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc hendrerit est elementum mi venenatis vehicula. Sed vitae accumsan urna, nec maximus justo. Proin sit amet dui enim. Nullam nunc arcu, elementum at eros eu, fringilla luctus neque. Nam varius lacus nec quam viverra, eu blandit mi dapibus. Curabitur vel vehicula mauris. Maecenas interdum ante et sollicitudin scelerisque. Quisque viverra metus ac diam consectetur lobortis. Aenean blandit egestas ipsum, ultricies suscipit lectus efficitur in. Sed dictum aliquam diam, ut vulputate sem imperdiet porttitor. Sed et pellentesque nulla. Ut ac ultrices felis. Donec vulputate, ligula nec volutpat ultricies, mi augue pulvinar ipsum, ac pretium diam erat vel nunc. Sed sit amet libero et dolor tempor gravida eget vel mauris.
Nullam aliquam, mi ac dapibus condimentum, ligula dolor accumsan mauris, eu ultricies felis felis ut risus. Etiam iaculis odio in mauris euismod vulputate. Maecenas eget ligula cursus, condimentum mi sed, tristique metus. Maecenas semper diam at iaculis faucibus. Sed non urna volutpat, convallis augue eu, faucibus ex. Proin fringilla quam lacus, id tristique ipsum imperdiet at. Donec cursus ex feugiat, fringilla sem in, porta ipsum.

Fusce at leo et nisl maximus dictum. Maecenas at lacinia felis, ac pellentesque metus. Nullam luctus maximus urna eu blandit. Cras interdum at libero vitae bibendum. Praesent quis felis aliquet, congue lorem non, tempus lectus. Nunc luctus pellentesque rutrum. Mauris accumsan molestie tortor a dapibus. Sed efficitur eu libero quis iaculis. Mauris bibendum metus lectus. Nunc pellentesque velit odio, ut sagittis nisl hendrerit ac. Ut eu lectus ultrices, vestibulum odio eget, volutpat purus. Nullam id ultrices mi. Maecenas in lorem consectetur nulla bibendum ornare.

Nam suscipit aliquam justo non lacinia. Nullam ligula purus, euismod sed porta eu, interdum a quam. Sed consectetur risus mi, ac porttitor arcu suscipit nec. Vestibulum et tellus odio. Aliquam nec facilisis felis, eget blandit nulla. Vivamus elementum porta molestie. Fusce id gravida sem, eget faucibus massa. Morbi iaculis ultrices congue. Ut mattis ac augue dapibus dignissim.

Donec ullamcorper, arcu vitae malesuada malesuada, quam ante tincidunt ligula, nec luctus nunc odio pharetra arcu. Nunc est mauris, feugiat vitae elementum nec, efficitur sit amet odio. Nunc tempor velit a velit pulvinar, quis eleifend mi lacinia. Mauris placerat augue in pretium consequat. In egestas varius massa at ultrices. Praesent laoreet semper enim. Aenean eu tortor consectetur, laoreet urna eget, lacinia mauris. Ut malesuada lorem at bibendum finibus. Ut rhoncus ante est, in cursus nibh ultrices id. Suspendisse fringilla dolor tellus, ac placerat urna tristique at.


Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc hendrerit est elementum mi venenatis vehicula. Sed vitae accumsan urna, nec maximus justo. Proin sit amet dui enim. Nullam nunc arcu, elementum at eros eu, fringilla luctus neque. Nam varius lacus nec quam viverra, eu blandit mi dapibus. Curabitur vel vehicula mauris. Maecenas interdum ante et sollicitudin scelerisque. Quisque viverra metus ac diam consectetur lobortis. Aenean blandit egestas ipsum, ultricies suscipit lectus efficitur in. Sed dictum aliquam diam, ut vulputate sem imperdiet porttitor. Sed et pellentesque nulla. Ut ac ultrices felis. Donec vulputate, ligula nec volutpat ultricies, mi augue pulvinar ipsum, ac pretium diam erat vel nunc. Sed sit amet libero et dolor tempor gravida eget vel mauris.

Nullam aliquam, mi ac dapibus condimentum, ligula dolor accumsan mauris, eu ultricies felis felis ut risus. Etiam iaculis odio in mauris euismod vulputate. Maecenas eget ligula cursus, condimentum mi sed, tristique metus. Maecenas semper diam at iaculis faucibus. Sed non urna volutpat, convallis augue eu, faucibus ex. Proin fringilla quam lacus, id tristique ipsum imperdiet at. Donec cursus ex feugiat, fringilla sem in, porta ipsum.

Fusce at leo et nisl maximus dictum. Maecenas at lacinia felis, ac pellentesque metus. Nullam luctus maximus urna eu blandit. Cras interdum at libero vitae bibendum. Praesent quis felis aliquet, congue lorem non, tempus lectus. Nunc luctus pellentesque rutrum. Mauris accumsan molestie tortor a dapibus. Sed efficitur eu libero quis iaculis. Mauris bibendum metus lectus. Nunc pellentesque velit odio, ut sagittis nisl hendrerit ac. Ut eu lectus ultrices, vestibulum odio eget, volutpat purus. Nullam id ultrices mi. Maecenas in lorem consectetur nulla bibendum ornare.

Nam suscipit aliquam justo non lacinia. Nullam ligula purus, euismod sed porta eu, interdum a quam. Sed consectetur risus mi, ac porttitor arcu suscipit nec. Vestibulum et tellus odio. Aliquam nec facilisis felis, eget blandit nulla. Vivamus elementum porta molestie. Fusce id gravida sem, eget faucibus massa. Morbi iaculis ultrices congue. Ut mattis ac augue dapibus dignissim.

Donec ullamcorper, arcu vitae malesuada malesuada, quam ante tincidunt ligula, nec luctus nunc odio pharetra arcu. Nunc est mauris, feugiat vitae elementum nec, efficitur sit amet odio. Nunc tempor velit a velit pulvinar, quis eleifend mi lacinia. Mauris placerat augue in pretium consequat. In egestas varius massa at ultrices. Praesent laoreet semper enim. Aenean eu tortor consectetur, laoreet urna eget, lacinia mauris. Ut malesuada lorem at bibendum finibus. Ut rhoncus ante est, in cursus nibh ultrices id. Suspendisse fringilla dolor tellus, ac placerat urna tristique at.


Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc hendrerit est elementum mi venenatis vehicula. Sed vitae accumsan urna, nec maximus justo. Proin sit amet dui enim. Nullam nunc arcu, elementum at eros eu, fringilla luctus neque. Nam varius lacus nec quam viverra, eu blandit mi dapibus. Curabitur vel vehicula mauris. Maecenas interdum ante et sollicitudin scelerisque. Quisque viverra metus ac diam consectetur lobortis. Aenean blandit egestas ipsum, ultricies suscipit lectus efficitur in. Sed dictum aliquam diam, ut vulputate sem imperdiet porttitor. Sed et pellentesque nulla. Ut ac ultrices felis. Donec vulputate, ligula nec volutpat ultricies, mi augue pulvinar ipsum, ac pretium diam erat vel nunc. Sed sit amet libero et dolor tempor gravida eget vel mauris.

Nullam aliquam, mi ac dapibus condimentum, ligula dolor accumsan mauris, eu ultricies felis felis ut risus. Etiam iaculis odio in mauris euismod vulputate. Maecenas eget ligula cursus, condimentum mi sed, tristique metus. Maecenas semper diam at iaculis faucibus. Sed non urna volutpat, convallis augue eu, faucibus ex. Proin fringilla quam lacus, id tristique ipsum imperdiet at. Donec cursus ex feugiat, fringilla sem in, porta ipsum.

Fusce at leo et nisl maximus dictum. Maecenas at lacinia felis, ac pellentesque metus. Nullam luctus maximus urna eu blandit. Cras interdum at libero vitae bibendum. Praesent quis felis aliquet, congue lorem non, tempus lectus. Nunc luctus pellentesque rutrum. Mauris accumsan molestie tortor a dapibus. Sed efficitur eu libero quis iaculis. Mauris bibendum metus lectus. Nunc pellentesque velit odio, ut sagittis nisl hendrerit ac. Ut eu lectus ultrices, vestibulum odio eget, volutpat purus. Nullam id ultrices mi. Maecenas in lorem consectetur nulla bibendum ornare.

Nam suscipit aliquam justo non lacinia. Nullam ligula purus, euismod sed porta eu, interdum a quam. Sed consectetur risus mi, ac porttitor arcu suscipit nec. Vestibulum et tellus odio. Aliquam nec facilisis felis, eget blandit nulla. Vivamus elementum porta molestie. Fusce id gravida sem, eget faucibus massa. Morbi iaculis ultrices congue. Ut mattis ac augue dapibus dignissim.

Donec ullamcorper, arcu vitae malesuada malesuada, quam ante tincidunt ligula, nec luctus nunc odio pharetra arcu. Nunc est mauris, feugiat vitae elementum nec, efficitur sit amet odio. Nunc tempor velit a velit pulvinar, quis eleifend mi lacinia. Mauris placerat augue in pretium consequat. In egestas varius massa at ultrices. Praesent laoreet semper enim. Aenean eu tortor consectetur, laoreet urna eget, lacinia mauris. Ut malesuada lorem at bibendum finibus. Ut rhoncus ante est, in cursus nibh ultrices id. Suspendisse fringilla dolor tellus, ac placerat urna tristique at.


Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc hendrerit est elementum mi venenatis vehicula. Sed vitae accumsan urna, nec maximus justo. Proin sit amet dui enim. Nullam nunc arcu, elementum at eros eu, fringilla luctus neque. Nam varius lacus nec quam viverra, eu blandit mi dapibus. Curabitur vel vehicula mauris. Maecenas interdum ante et sollicitudin scelerisque. Quisque viverra metus ac diam consectetur lobortis. Aenean blandit egestas ipsum, ultricies suscipit lectus efficitur in. Sed dictum aliquam diam, ut vulputate sem imperdiet porttitor. Sed et pellentesque nulla. Ut ac ultrices felis. Donec vulputate, ligula nec volutpat ultricies, mi augue pulvinar ipsum, ac pretium diam erat vel nunc. Sed sit amet libero et dolor tempor gravida eget vel mauris.

Nullam aliquam, mi ac dapibus condimentum, ligula dolor accumsan mauris, eu ultricies felis felis ut risus. Etiam iaculis odio in mauris euismod vulputate. Maecenas eget ligula cursus, condimentum mi sed, tristique metus. Maecenas semper diam at iaculis faucibus. Sed non urna volutpat, convallis augue eu, faucibus ex. Proin fringilla quam lacus, id tristique ipsum imperdiet at. Donec cursus ex feugiat, fringilla sem in, porta ipsum.

Fusce at leo et nisl maximus dictum. Maecenas at lacinia felis, ac pellentesque metus. Nullam luctus maximus urna eu blandit. Cras interdum at libero vitae bibendum. Praesent quis felis aliquet, congue lorem non, tempus lectus. Nunc luctus pellentesque rutrum. Mauris accumsan molestie tortor a dapibus. Sed efficitur eu libero quis iaculis. Mauris bibendum metus lectus. Nunc pellentesque velit odio, ut sagittis nisl hendrerit ac. Ut eu lectus ultrices, vestibulum odio eget, volutpat purus. Nullam id ultrices mi. Maecenas in lorem consectetur nulla bibendum ornare.

Nam suscipit aliquam justo non lacinia. Nullam ligula purus, euismod sed porta eu, interdum a quam. Sed consectetur risus mi, ac porttitor arcu suscipit nec. Vestibulum et tellus odio. Aliquam nec facilisis felis, eget blandit nulla. Vivamus elementum porta molestie. Fusce id gravida sem, eget faucibus massa. Morbi iaculis ultrices congue. Ut mattis ac augue dapibus dignissim.

Donec ullamcorper, arcu vitae malesuada malesuada, quam ante tincidunt ligula, nec luctus nunc odio pharetra arcu. Nunc est mauris, feugiat vitae elementum nec, efficitur sit amet odio. Nunc tempor velit a velit pulvinar, quis eleifend mi lacinia. Mauris placerat augue in pretium consequat. In egestas varius massa at ultrices. Praesent laoreet semper enim. Aenean eu tortor consectetur, laoreet urna eget, lacinia mauris. Ut malesuada lorem at bibendum finibus. Ut rhoncus ante est, in cursus nibh ultrices id. Suspendisse fringilla dolor tellus, ac placerat urna tristique at.


Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc hendrerit est elementum mi venenatis vehicula. Sed vitae accumsan urna, nec maximus justo. Proin sit amet dui enim. Nullam nunc arcu, elementum at eros eu, fringilla luctus neque. Nam varius lacus nec quam viverra, eu blandit mi dapibus. Curabitur vel vehicula mauris. Maecenas interdum ante et sollicitudin scelerisque. Quisque viverra metus ac diam consectetur lobortis. Aenean blandit egestas ipsum, ultricies suscipit lectus efficitur in. Sed dictum aliquam diam, ut vulputate sem imperdiet porttitor. Sed et pellentesque nulla. Ut ac ultrices felis. Donec vulputate, ligula nec volutpat ultricies, mi augue pulvinar ipsum, ac pretium diam erat vel nunc. Sed sit amet libero et dolor tempor gravida eget vel mauris.

Nullam aliquam, mi ac dapibus condimentum, ligula dolor accumsan mauris, eu ultricies felis felis ut risus. Etiam iaculis odio in mauris euismod vulputate. Maecenas eget ligula cursus, condimentum mi sed, tristique metus. Maecenas semper diam at iaculis faucibus. Sed non urna volutpat, convallis augue eu, faucibus ex. Proin fringilla quam lacus, id tristique ipsum imperdiet at. Donec cursus ex feugiat, fringilla sem in, porta ipsum.

Fusce at leo et nisl maximus dictum. Maecenas at lacinia felis, ac pellentesque metus. Nullam luctus maximus urna eu blandit. Cras interdum at libero vitae bibendum. Praesent quis felis aliquet, congue lorem non, tempus lectus. Nunc luctus pellentesque rutrum. Mauris accumsan molestie tortor a dapibus. Sed efficitur eu libero quis iaculis. Mauris bibendum metus lectus. Nunc pellentesque velit odio, ut sagittis nisl hendrerit ac. Ut eu lectus ultrices, vestibulum odio eget, volutpat purus. Nullam id ultrices mi. Maecenas in lorem consectetur nulla bibendum ornare.

Nam suscipit aliquam justo non lacinia. Nullam ligula purus, euismod sed porta eu, interdum a quam. Sed consectetur risus mi, ac porttitor arcu suscipit nec. Vestibulum et tellus odio. Aliquam nec facilisis felis, eget blandit nulla. Vivamus elementum porta molestie. Fusce id gravida sem, eget faucibus massa. Morbi iaculis ultrices congue. Ut mattis ac augue dapibus dignissim.

Donec ullamcorper, arcu vitae malesuada malesuada, quam ante tincidunt ligula, nec luctus nunc odio pharetra arcu. Nunc est mauris, feugiat vitae elementum nec, efficitur sit amet odio. Nunc tempor velit a velit pulvinar, quis eleifend mi lacinia. Mauris placerat augue in pretium consequat. In egestas varius massa at ultrices. Praesent laoreet semper enim. Aenean eu tortor consectetur, laoreet urna eget, lacinia mauris. Ut malesuada lorem at bibendum finibus. Ut rhoncus ante est, in cursus nibh ultrices id. Suspendisse fringilla dolor tellus, ac placerat urna tristique at.";
    }

    fn variables(&self) -> Vec<String> {
        return vec!["x".to_owned()];
    }

    fn solve(&self, _variables: HashMap<&str, &str>) -> Result<String, ChallengeError> {
        return Ok("Answer".to_string());
    }
}
