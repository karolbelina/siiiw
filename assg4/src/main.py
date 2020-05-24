import argparse
from sklearn.model_selection import train_test_split
from stop_words import get_stop_words
from sklearn.feature_extraction.text import CountVectorizer
from sklearn.feature_selection import chi2, SelectKBest
from sklearn.naive_bayes import MultinomialNB
from sklearn.tree import DecisionTreeClassifier
from sklearn.model_selection import cross_val_score, KFold
from sklearn.metrics import classification_report, confusion_matrix, accuracy_score
import numpy as np


def load(directory: str, n=None) -> (list, list):
    import os

    root, _, filenames = next(os.walk(directory, topdown=False))
    if n is not None:
        filenames = filenames[:n]

    def extract(filename):
        category = filename.split('_')[0]
        with open(os.path.join(os.getcwd(), root, filename), 'r') as file:
            article = file.read()
            return article, category

    articles, categories = zip(*map(extract, filenames))
    return np.array(articles), np.array(categories)


parser = argparse.ArgumentParser()
parser.add_argument('directory', help='Dataset directory')
args = parser.parse_args()

print("Loading data... ", end='', flush=True)
articles, categories = load(args.directory)
train_articles, test_articles, train_categories, test_categories = \
    train_test_split(articles, categories, test_size=0.2, random_state=0)
print("done")

print("Extracting and selecting features... ", end='', flush=True)
vectorizer = CountVectorizer(analyzer='word', stop_words=get_stop_words('polish'), max_df=0.9)
vectorizer.fit(train_articles)
# analyze = vectorizer.build_analyzer()
# print(analyze(articles[0]))
train_features = vectorizer.transform(train_articles)
test_features = vectorizer.transform(test_articles)

selector = SelectKBest(chi2, k=10000) # 229803
selector.fit(train_features, train_categories)
selected_train_features = selector.transform(train_features)
selected_test_features = selector.transform(test_features)
print("done")

models = [
    MultinomialNB(alpha=0.01),
    DecisionTreeClassifier(criterion='gini')
]

for model in models:
    print(f"Training {type(model).__name__}... ", end='', flush=True)
    kfold = KFold(n_splits=10, shuffle=True)

    def fold(model, train_indexes, validation_indexes):
        model.fit(selected_train_features[train_indexes], train_categories[train_indexes])
        score = model.score(selected_train_features[validation_indexes], train_categories[validation_indexes])
        return model, score

    folds = [fold(model, train_indexes, validation_indexes)
        for train_indexes, validation_indexes in kfold.split(selected_train_features, train_categories)]
    best_model = max(folds, key=lambda fold: fold[1])[0]
    print("done")

    print(f"Evaluating {type(best_model).__name__}... ", end='', flush=True)
    predicted_categories = best_model.predict(selected_test_features)
    accuracy = accuracy_score(test_categories, predicted_categories)
    print(f"done, accuracy = {accuracy:.2%}")
